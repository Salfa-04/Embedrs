use crate::hal::{self, bind_interrupts, peripherals};

bind_interrupts! {
    pub struct IntRqst {
        USART3 => hal::usart::InterruptHandler<peripherals::USART3>;

        // USART1 => hal::usart::InterruptHandler<peripherals::USART1>;
        // USART2 => hal::usart::InterruptHandler<peripherals::USART2>;
        // USART6 => hal::usart::InterruptHandler<peripherals::USART6>;

        // CAN1_TX => hal::can::TxInterruptHandler<peripherals::CAN1>;
        // CAN1_RX0 => hal::can::Rx0InterruptHandler<peripherals::CAN1>;
        // CAN1_RX1 => hal::can::Rx1InterruptHandler<peripherals::CAN1>;
        // CAN1_SCE => hal::can::SceInterruptHandler<peripherals::CAN1>;

        // CAN2_TX => hal::can::TxInterruptHandler<peripherals::CAN2>;
        // CAN2_RX0 => hal::can::Rx0InterruptHandler<peripherals::CAN2>;
        // CAN2_RX1 => hal::can::Rx1InterruptHandler<peripherals::CAN2>;
        // CAN2_SCE => hal::can::SceInterruptHandler<peripherals::CAN2>;

    }
}
