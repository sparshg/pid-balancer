# pid-balancer
A Proportional-Integral-Derivative controller to self balance a ball

Physics for the simulation is implemented according to [this paper](https://www.academia.edu/76867878/Swing_up_and_positioning_control_of_an_inverted_wheeled_cart_pendulum_system_with_chaotic_balancing_motions) (excluding the counter-balances and connecting rod)

I used Range-Kutta method (4th order) to solve the system.
Camera dynamics are implemented with the help of [this](https://www.youtube.com/watch?v=KPoeNZZ6H4s) video

Use arrow keys to control the cart, and disturb the ball.



<img width="1392" alt="Screenshot 2023-04-27 at 8 30 20 PM 2" src="https://user-images.githubusercontent.com/43041139/234903949-43b97b14-b21a-4951-9278-aef4930595bc.png">
