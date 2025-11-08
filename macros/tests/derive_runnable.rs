use caretta_sync_core::RunnableCommand;
use caretta_sync_macros::RunnableCommand;

struct RunnableCommandStruct1;

impl RunnableCommand for RunnableCommandStruct1 {
    fn run(self, app_name: &'static str) {
        print!("Run {}", stringify!(RunnableCommandStruct1::run()))
    }
}

#[derive(RunnableCommand)]
enum RunnableCommandEnum {
    Struct1(RunnableCommandStruct1),
}

#[derive(RunnableCommand)]
struct RunnableCommandStruct2 {
    #[runnable_command]
    runnable: RunnableCommandEnum,
}

#[tokio::test]
async fn test() {
    let runnable = RunnableCommandStruct2 {
        runnable: RunnableCommandEnum::Struct1(RunnableCommandStruct1),
    };
    runnable.run("runnable_app");
}
