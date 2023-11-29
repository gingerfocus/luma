macro_rules! get_luma {
    () => {
        futures::executor::block_on(async { $crate::prelude::LUMA.read().await })
    };
}
pub(crate) use get_luma;

// macro_rules! get_mut_luma {
//     () => {
//         futures::executor::block_on(async { $crate::prelude::LUMA.write().await })
//     };
// }
//
// pub(crate) use get_mut_luma;

macro_rules! get_state_mut {
    () => {
        futures::executor::block_on(async { $crate::prelude::STATE.write().await })
    };
}
pub(crate) use get_state_mut;
