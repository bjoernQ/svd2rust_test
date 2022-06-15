fn main() {
    let mut data = 0u32;
    let data_ptr = &mut data as *mut _ as *mut u32;
    unsafe {
        &*(data_ptr as *mut generated23::generic::Reg<generated23::rmt::int_clr::INT_CLR_SPEC>)
    }
    .write(|w| unsafe { w.ch_tx_thr_event_int_clr(7).set_bit() });
    println!("With svd2rust 0.23.0: {:032b}", data);

    data = 0u32;
    unsafe {
        &*(data_ptr as *mut generated24::generic::Reg<generated24::rmt::int_clr::INT_CLR_SPEC>)
    }
    .write(|w| unsafe { w.ch_tx_thr_event_int_clr::<7>().set_bit() });
    println!("With svd2rust 0.24.0: {:032b}", data);
}
