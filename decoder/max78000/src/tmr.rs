use hal::gcr::{ClockForPeripheral, GcrRegisters};
use hal::pac;

pub struct Tmr2 {
    tmr: pac::Tmr2,
}

impl Tmr2
{
    pub fn new(tmr: pac::Tmr2, reg: &mut GcrRegisters) -> Self {
        unsafe { tmr.enable_clock(&mut reg.gcr) };
        Self { tmr }
    }

    /// Get the current tick count
    pub fn get_tick_count(&self) -> u32 {
        return self.tmr.cnt().read().bits();
    }

    /// Configure TMR2
    /// - PCLK
    /// - Continuous mode
    /// - 32-bit cascade mode
    /// - Range: 0x1 to 0xFFFFFFFF
    pub fn config(&self) {
        self.disable();
        // Set the tmr2 clock source to the PCLK
        // Safety: The clksel_a field is 2 bits wide, which fits the value 0b00
        self.tmr.ctrl1().modify(|_, w| unsafe {
            w.clksel_a().bits(0b00)
        });
        // Set mode to cascade
        self.tmr.ctrl1().modify(|_, w| w.cascade().set_bit());
        // Set 32-bit cascade mode
        self.tmr.ctrl0().modify(|_, w| w
            .pol_a().clear_bit()        // Set polarity to active high
            .clkdiv_a().div_by_1()      // Set prescaler to divide by 1
            .mode_a().continuous()      // Set continuous mode
        );
        // Set initial count to 0x1
        self.tmr.cnt().write(|w| unsafe { w.bits(0x1) });
        // Set the compare value to 0xFFFFFFFF
        self.tmr.cmp().write(|w| unsafe { w.bits(0xFFFFFFFF) });
        // Enable timer clock source
        self.tmr.ctrl0().modify(|_, w| w.clken_a().set_bit());
        while self.tmr.ctrl1().read().clkrdy_a().bit_is_clear() { }
        // Enable timer
        self.enable();
    }

    /// Disable the TMR peripheral.
    fn disable(&self) {
        self.tmr.ctrl0().modify(|_, w| w
            .en_a().clear_bit()
            .en_b().clear_bit()
        );
        while self.tmr.ctrl1().read().clken_a().bit_is_set() { }
        while self.tmr.ctrl1().read().clken_b().bit_is_set() { }
    }

    /// Enable the TMR peripheral.
    fn enable(&self) {
        self.tmr.ctrl0().modify(|_, w| w
            .en_a().set_bit()
        );
        while self.tmr.ctrl0().read().clken_a().bit_is_clear() { }
    }
}
