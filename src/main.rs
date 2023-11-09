
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs")); 

//use core::panic::PanicInfo;

use cortex_m::{
    self,       
};

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use stm32_hal2::{
    self,
    clocks::Clocks,
    dma::{Dma, DmaChannel, DmaInterrupt},
    gpio::{Edge, OutputType, Pin, PinMode, Port, Pull},
    low_power, pac,
    timer::{Timer, TimerInterrupt},
};


#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

const GPIOC: *mut GPIO_TypeDef = GPIOC_BASE as *mut GPIO_TypeDef;
const GPIO_PIN_6: u16 = 0x0040;
//const adcValue: i32 = 0;
const Delay : u32 = 1000;


#[entry]
fn main() -> ! {
    unsafe {
        /*
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = pac::Peripherals::take().unwrap();
        stm32_hal2::debug_workaround();
        let clock_cfg = Clocks::default();
        clock_cfg.setup().unwrap();
        */

        // Initialize the HAL Library
        HAL_Init();
        
        // Configure the system clock
        SystemClock_Config();
        
        // Initialize all configured peripherals
        MX_GPIO_Init();  
        MX_ADC1_Init();
        MX_USART2_UART_Init();
           
        
        // Blink the LED in a loop
        loop {
            // LED is connected to pin 6 on port C
            HAL_GPIO_WritePin(GPIOC, GPIO_PIN_6, GPIO_PinState_GPIO_PIN_SET);
            defmt::println!("Output pin is low.");
            HAL_Delay(Delay);  

            HAL_GPIO_WritePin(GPIOC, GPIO_PIN_6, GPIO_PinState_GPIO_PIN_RESET);
            defmt::println!("Output pin is high.");
            HAL_Delay(Delay);  
        }
    }
}


////////////////////////////////////////////////////////////////////

/*
#![no_main]
#![no_std]

use cortex_m::{
    self,
    delay::Delay,
    //interrupt::{free, Mutex},
    //peripheral::NVIC,
};
use cortex_m_rt::entry;

// These lines are part of our setup for debug printing.
use defmt_rtt as _;
use panic_probe as _;

// Import parts of this library we use. You could use this style, or perhaps import
// less here.
use stm32_hal2::{
    self,
    clocks::Clocks,
    dma::{Dma, DmaChannel, DmaInterrupt},
    gpio::{Edge, OutputType, Pin, PinMode, Port, Pull},
    low_power, pac,
    timer::{Timer, TimerInterrupt},
};

#[entry]
fn main() -> ! {
    // Set up ARM Cortex-M peripherals. These are common to many MCUs, including all STM32 ones.
    let cp = cortex_m::Peripherals::take().unwrap();
    // Set up peripherals specific to the microcontroller you're using.
    let dp = pac::Peripherals::take().unwrap();

    // This line is required to prevent the debugger from disconnecting on entering WFI.
    // This appears to be a limitation of many STM32 families. Not required in production code,
    // and significantly increases power consumption in low-power modes.
    stm32_hal2::debug_workaround();

    // Create an initial clock configuration that uses the MCU's internal oscillator (HSI),
    // sets the MCU to its maximum system clock speed.
    let clock_cfg = Clocks::default();

    // Write the clock configuration to the MCU. If you wish, you can modify `clocks` above
    // in accordance with [its docs](https://docs.rs/stm32-hal2/0.2.0/stm32_hal2/clocks/index.html),
    // and the `clock_cfg` example.
    clock_cfg.setup().unwrap();

    // Setup a delay, based on the Cortex-m systick.
    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());
    let mut led = Pin::new(Port::C, 6, PinMode::Output);
    defmt::println!("Hello, world!");

    loop {
        led.set_low();
        defmt::println!("Output pin is low.");
        delay.delay_ms(1_000);

        led.set_high();
        defmt::println!("Output pin is high.");
        delay.delay_ms(1_000);
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
*/