# PID Controller Simualation
A Proportional-Integral-Derivative controller to self balance a ball on a rolling cart. Use arrow keys to control the cart, and disturb the ball.

## Try on Web
https://sparshg.github.io/pid-balancer/

## Downloads for Desktop

Windows, Mac: https://github.com/sparshg/pid-balancer/releases

(Should work on Linux too, didn't compile)


## Implementation Details

Physics for the simulation is implemented according to [this paper](https://www.academia.edu/76867878/Swing_up_and_positioning_control_of_an_inverted_wheeled_cart_pendulum_system_with_chaotic_balancing_motions) (excluding the counter-balances and connecting rod)

I used Runge-Kutta method (4th order) to solve the system. System's energy will remain almost constant when controller is off and there is no drag.

Camera dynamics are implemented with the help of [this](https://www.youtube.com/watch?v=KPoeNZZ6H4s) video


<img width="1440" alt="demo" src="https://github.com/sparshg/pid-balancer/assets/43041139/0f4e9d4b-115e-437e-82c4-6cbdcc593822">
