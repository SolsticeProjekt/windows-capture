use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use std::sync::{Arc, Mutex, atomic::{AtomicBool}};
use std::thread::{self, JoinHandle};
use std::sync::mpsc;
use windows::Win32::System::Com::CoIncrementMTAUsage;

// Replace your static OnceLock with PyOnceLock:
static INIT_MTA: PyOnceLock<()> = PyOnceLock::new();

impl<T: GraphicsCaptureApiHandler + Send + 'static, E> CaptureControl<T, E> {

    // ...

    fn start_free_threaded<T: TryInto<GraphicsCaptureItemType> + Send + 'static>(
        settings: Settings<Self::Flags, T>,
    ) -> Result<CaptureControl<Self, Self::Error>, GraphicsCaptureApiError<Self::Error>>
    where
        Self: Send + 'static,
        <Self as GraphicsCaptureApiHandler>::Flags: Send,
    {
        let (halt_sender, halt_receiver) = mpsc::channel::<Arc<AtomicBool>>();
        let (callback_sender, callback_receiver) = mpsc::channel::<Arc<Mutex<Self>>>();

        let thread_handle = thread::spawn(move || -> Result<(), GraphicsCaptureApiError<Self::Error>> {
            Python::with_gil(|py| {
                // Initialize with PyOnceLock and Python GIL token `py`
                INIT_MTA.get_or_init(py, || {
                    unsafe {
                        CoIncrementMTAUsage().expect("Failed to increment MTA usage");
                    }
                });
                Ok(())
            })?;

            // ... rest of your initialization and capture start code ...

            Ok(())
        });

        // ... rest of your control setup and return ...

        // Example simplified:
        let halt_handle = halt_receiver.recv().map_err(|_| GraphicsCaptureApiError::FailedToJoinThread)?;
        let callback = callback_receiver.recv().map_err(|_| GraphicsCaptureApiError::FailedToJoinThread)?;

        Ok(CaptureControl::new(thread_handle, halt_handle, callback))
    }
}
