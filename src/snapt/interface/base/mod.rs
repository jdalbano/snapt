pub trait InterfaceBase {
    fn new() -> Self;
    fn init(&mut self);   
    fn check_should_close(&self) -> bool; 
    fn handle_messages(&self) -> bool;
    fn destroy(&mut self);

    fn run(&mut self) {
        self.init();
        self.start_message_handler();
        self.destroy();
    }

    fn start_message_handler(&self) {        
        loop {
            if self.check_should_close() {
                break;
            }

            let was_message_handled = self.handle_messages();

            if !was_message_handled {
                break;
            }
        }
    }
}
