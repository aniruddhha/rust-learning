mod mobile;
mod tv;
mod simple_module;

use simple_module::abc::hi_abc;

fn main() {
    mobile::android::call_person();

    hi_abc();
}
