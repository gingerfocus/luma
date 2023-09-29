macro_rules! get_luma {
    () => {
        futures::executor::block_on(async { LUMA.read().await })
    };
}
pub(crate) use get_luma;

#[allow(unused)]
macro_rules! get_mut_luma {
    () => {
        futures::executor::block_on(async { LUMA.write().await })
    };
}
#[allow(unused)]
pub(crate) use get_mut_luma;
