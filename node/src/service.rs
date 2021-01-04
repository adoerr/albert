use std::sync::Arc;

use sc_consensus_manual_seal::InstantSealParams;
use sc_executor::native_executor_instance;
use sc_service::{error::Error as ServiceError, Configuration, PartialComponents, TaskManager};
use sp_api::TransactionFor;
use sp_consensus::import_queue::BasicQueue;

use albert_runtime::{self, opaque::Block, RuntimeApi};

pub use sc_executor::NativeExecutor;

native_executor_instance!(
    pub Executor,
    albert_runtime::api::dispatch,
    albert_runtime::native_version,
    frame_benchmarking::benchmarking::HostFunctions,
);

type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectedChain = sc_consensus::LongestChain<FullBackend, Block>;

pub fn new_partial(
    config: &Configuration,
) -> Result<
    PartialComponents<
        FullClient,
        FullBackend,
        FullSelectedChain,
        BasicQueue<Block, TransactionFor<FullClient, Block>>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (),
    >,
    ServiceError,
> {
    // create full node initial parts
    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config)?;

    let client = Arc::new(client);

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        None,
        task_manager.spawn_handle(),
        client.clone(),
    );

    let inherent_data_providers = sp_inherents::InherentDataProviders::new();
    inherent_data_providers
        .register_provider(sp_timestamp::InherentDataProvider)
        .map_err(sp_consensus::error::Error::InherentData)?;

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let import_queue = sc_consensus_manual_seal::import_queue(
        Box::new(client.clone()),
        &task_manager.spawn_handle(),
        None,
    );

    Ok(PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        inherent_data_providers,
        other: (),
    })
}

/// Bootstrap services for a new full client
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
    let PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        inherent_data_providers,
        other: (),
    } = new_partial(&config)?;

    let (network, network_status_sinks, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            on_demand: None,
            block_announce_validator_builder: None,
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config,
            backend.clone(),
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let role = config.role.clone();
    let telemtry_connection_sinks = sc_service::TelemetryConnectionSinks::default();

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
            };

            crate::rpc::create_full(deps)
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: network.clone(),
        client: client.clone(),
        keystore: keystore_container.sync_keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        telemetry_connection_sinks: telemtry_connection_sinks.clone(),
        rpc_extensions_builder,
        on_demand: None,
        remote_blockchain: None,
        backend,
        network_status_sinks,
        system_rpc_tx,
        config,
    })?;

    if role.is_authority() {
        let proposer = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            None,
        );

        let instant_seal = sc_consensus_manual_seal::run_instant_seal(InstantSealParams {
            block_import: client.clone(),
            env: proposer,
            client,
            pool: transaction_pool.pool().clone(),
            select_chain,
            consensus_data_provider: None,
            inherent_data_providers,
        });

        // the AURA authoring task is considered essential, i.e. if it fails we take down
        // the service itself as well.
        task_manager
            .spawn_essential_handle()
            .spawn_blocking("instant-seal", instant_seal);
    }

    network_starter.start_network();

    Ok(task_manager)
}
