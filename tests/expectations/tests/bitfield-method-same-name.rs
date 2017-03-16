/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Foo {
    pub _bitfield_1: u8,
    pub __bindgen_align: [u8; 0usize],
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Foo ) ));
    assert_eq! (::std::mem::align_of::<Foo>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Foo ) ));
}
extern "C" {
    #[link_name = "_ZN3Foo4typeEv"]
    pub fn Foo_type(this: *mut Foo) -> ::std::os::raw::c_schar;
}
extern "C" {
    #[link_name = "_ZN3Foo9set_type_Ec"]
    pub fn Foo_set_type_(this: *mut Foo, c: ::std::os::raw::c_schar);
}
extern "C" {
    #[link_name = "_ZN3Foo8set_typeEc"]
    pub fn Foo_set_type(this: *mut Foo, c: ::std::os::raw::c_schar);
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}
impl Foo {
    #[inline]
    pub fn type__bindgen_bitfield(&self) -> ::std::os::raw::c_schar {
        let mask = 7usize as u8;
        let field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u8) }
    }
    #[inline]
    pub fn set_type__bindgen_bitfield(&mut self,
                                      val: ::std::os::raw::c_schar) {
        let mask = 7usize as u8;
        let val = val as u8 as u8;
        let mut field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        field_val &= !mask;
        field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
    }
    #[inline]
    pub unsafe fn type_(&mut self) -> ::std::os::raw::c_schar {
        Foo_type(&mut *self)
    }
    #[inline]
    pub unsafe fn set_type_(&mut self, c: ::std::os::raw::c_schar) {
        Foo_set_type_(&mut *self, c)
    }
    #[inline]
    pub unsafe fn set_type(&mut self, c: ::std::os::raw::c_schar) {
        Foo_set_type(&mut *self, c)
    }
}
