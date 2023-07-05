use objc2::foundation::NSObject;
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct UIResponder;

    unsafe impl ClassType for UIResponder {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl UIResponder {
        #[sel(isFirstResponder)]
        fn __is_first_responder(&self) -> bool;

        #[sel(becomeFirstResponder)]
        fn __become_first_responder(&self) -> bool;

        #[sel(resignFirstResponder)]
        fn __resign_first_responder(&self) -> bool;
    }
);

impl UIResponder {
    pub fn is_first_responder(&self) -> bool {
        unsafe { self.__is_first_responder() }
    }

    pub fn become_first_responder(&self) -> bool {
        unsafe { self.__become_first_responder() }
    }

    pub fn resign_first_responder(&self) -> bool {
        unsafe { self.__resign_first_responder() }
    }
}
