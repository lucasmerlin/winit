use objc2::{extern_class, extern_methods, foundation::{NSObject, NSString}, ClassType};
use objc2::rc::{Id, Shared};
use objc2::msg_send_id;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSUserActivity;

    unsafe impl ClassType for NSUserActivity {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSUserActivity {
        pub fn webpageURL(&self) -> Option<Id<NSURL, Shared>> {
            unsafe { msg_send_id![self, webpageURL] }
        }
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSURL;

    unsafe impl ClassType for NSURL {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSURL {
        pub fn absoluteString(&self) -> Id<NSString, Shared> {
            unsafe { msg_send_id![self, absoluteString] }
        }
    }
);
