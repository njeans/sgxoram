use criterion::{criterion_group, criterion_main, Criterion};
use mc_fog_ledger_enclave::{LedgerSgxEnclave, ENCLAVE_FILE, LedgerEnclave};
use mc_common::{
    logger::{create_app_logger, log, o},
    //time::SystemTimeProvider,
    ResponderId,
};
use std::{env};
use std::vec::Vec;

pub fn criterion_benchmark(c: &mut Criterion) {
    mc_common::setup_panic_handler();
    let (logger, _global_logger_guard) = create_app_logger(o!());
    let enclave_path = env::current_exe()
        .expect("Could not get the path of our executable")
        .with_file_name(ENCLAVE_FILE);
    //let responder_id = ResponderId("123.123.123.0:9004".to_string());
    let enclave = LedgerSgxEnclave::new(
        enclave_path,
        //&responder_id,
        65536,
        logger.clone(),
    );

    //must be 32
    let key: Vec<u8> = Vec::from([2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3,5,2,3]);
    //must be 16
    //let value: Vec<u8> = Vec::from([4,6,121,42,4,6,121,42,4,6,121,42,4,6,121,42]);
    let value: Vec<u8> = Vec::from([2; 32]);
    //c.bench_function("omap add", |b| b.iter(|| enclave.add_omap_item(key.to_vec(), value.to_vec())));
    enclave.add_omap_item(key.to_vec(), value.to_vec()).unwrap();
    let retrieved_value = enclave.get_omap_item(key.to_vec()).unwrap();
    //c.bench_function("omap get", |b| b.iter(|| enclave.get_omap_item(key.to_vec())));
    assert_eq!(value.to_vec(), retrieved_value.to_vec());
    c.bench_function("add", |b| b.iter(|| 1+1));
    //let s = String::from_utf8(retrieved_value).unwrap();

    
    enclave.add_oram_item(0, Vec::from([2; 1024])).unwrap();
    assert_eq!(Vec::from([2; 1024]), enclave.get_oram_item(0).unwrap());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);