#!/bin/sh

cargo run STM32F405.svd RCC FLASH -USART6 +USART1 TIM1 TIM2 DMA2 | rustfmt > svd_stm32f405.rs

cargo run STM32F0x2.svd RCC USART1 +USART2 WWDG | rustfmt > svd_stm32f072.rs