use std::io::Read;

use rustyline::{DefaultEditor, ExternalPrinter};

use crate::error::Error;

pub struct ConsoleWriter {
    printer: Box<dyn ExternalPrinter + 'static + Send>
}

impl TryFrom<&mut DefaultEditor> for ConsoleWriter {
    type Error = Error;
    fn try_from(e: &mut DefaultEditor) -> Result<Self, Error> {
        Ok(ConsoleWriter {
            printer: Box::new(e.create_external_printer()?)
        })
    }

}

impl std::io::Write for ConsoleWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = String::from_utf8_lossy(buf).into_owned();
        let size = msg.as_bytes().len();
        self.printer.as_mut().print(msg);
        Ok(size)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}