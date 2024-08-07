use dread_engine::application::{Application, Definition};

fn main() {
    Application {
        application_definition: Definition {
            // The title that you would like to use for your application.
            title: "Dread Game".to_string(),

            // For now, there is no working directory usage, but you will ultimately be able to specify
            working_directory: "".to_string()
        }
    }.run();
}
