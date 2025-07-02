use lazy_supplements_core::utils::runnable::Runnable;
use lazy_supplements_macros::Runnable;

struct RunnableStruct1;

impl Runnable for RunnableStruct1 {
    async fn run(self) {
        print!("Run {}", stringify!(RunnableStruct1::run()))
    }
}

#[derive(Runnable)]
enum RunnableEnum {
    Struct1(RunnableStruct1),
}

#[derive(Runnable)]
struct RunnableStruct2 {
    #[runnable]
    runnable: RunnableEnum,
}

#[tokio::test]
async fn test() {
    let runnable = RunnableStruct2{
        runnable: RunnableEnum::Struct1(RunnableStruct1)
    };
    runnable.run().await;
}



