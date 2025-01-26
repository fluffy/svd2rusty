
.phony: all


all:
	- mkdir -p gen
	cargo build 
	cargo run STM32F0x2.svd  RCC FLASH WWDG USART1 +USART2 GPIOA +GPIOB  | rustfmt  > gen/svd_stm32f0x2.rs
	cargo run STM32F405.svd  RCC FLASH WWDG -USART6 +USART1 TIM1 TIM2 DMA2 GPIOA +GPIOB +GPIOC  | rustfmt  >  gen/svd_stm32f405.rs


