use jni::objects::GlobalRef;
use jni::{AttachGuard, JNIEnv, JavaVM};
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::async_runtime;
use tokio::runtime::{Builder, Runtime};

static CLASS_LOADER: OnceCell<GlobalRef> = OnceCell::new();
static JAVAVM: OnceCell<JavaVM> = OnceCell::new();
static RUNTIME: OnceCell<Runtime> = OnceCell::new();

#[derive(Debug, thiserror::Error)]
pub enum AndroidInitError {
    #[error("JVM not found")]
    JVMNotFound,
    #[error("JVM class loader")]
    JVMClassLoader,
    #[error(transparent)]
    JNI(#[from] jni::errors::Error),
}

std::thread_local! {
    static JNI_ENV: RefCell<Option<AttachGuard<'static>>> = RefCell::new(None);
}

pub fn create_runtime() -> Result<(), AndroidInitError> {
    let vm = JAVAVM.get().ok_or(AndroidInitError::JVMNotFound)?;
    let env = vm.attach_current_thread()?;

    setup_class_loader(&env)?;
    let runtime = {
        Builder::new_multi_thread()
            .enable_all()
            .thread_name_fn(|| {
                static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
                let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
                format!("intiface-thread-{}", id)
            })
            .on_thread_stop(move || {
                JNI_ENV.with(|f| *f.borrow_mut() = None);
            })
            .on_thread_start(move || {
                let vm = JAVAVM.get().unwrap();
                let env = vm.attach_current_thread().unwrap();

                let thread = env
                    .call_static_method(
                        "java/lang/Thread",
                        "currentThread",
                        "()Ljava/lang/Thread;",
                        &[],
                    )
                    .unwrap()
                    .l()
                    .unwrap();
                env.call_method(
                    thread,
                    "setContextClassLoader",
                    "(Ljava/lang/ClassLoader;)V",
                    &[CLASS_LOADER.get().unwrap().as_obj().into()],
                )
                .unwrap();
                JNI_ENV.with(|f| *f.borrow_mut() = Some(env));
            })
            .build()
            .unwrap()
    };
    let rh = runtime.handle().clone();
    RUNTIME.set(runtime).unwrap();
    async_runtime::set(rh);
    Ok(())
}

fn setup_class_loader(env: &JNIEnv) -> Result<(), AndroidInitError> {
    let thread = env
        .call_static_method(
            "java/lang/Thread",
            "currentThread",
            "()Ljava/lang/Thread;",
            &[],
        )?
        .l()?;
    let class_loader = env
        .call_method(
            thread,
            "getContextClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[],
        )?
        .l()?;

    CLASS_LOADER
        .set(env.new_global_ref(class_loader)?)
        .map_err(|_| AndroidInitError::JVMClassLoader)
}

#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, _res: *const std::os::raw::c_void) -> jni::sys::jint {
    let env = vm.get_env().unwrap();
    jni_utils::init(&env).unwrap();
    btleplug::platform::init(&env).unwrap();
    let _ = JAVAVM.set(vm);
    jni::JNIVersion::V6.into()
}
