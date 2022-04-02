```mermaid
graph TD
    Player
    Rocket
    Cannon
    Cannon-Ball
    City
    Timer
    Score

    Player -- spawns --> Rocket -- destroys --> Cannon-Ball
    Cannon -- spawns --> Cannon-Ball -- destroys --> City
    Timer -- increases --> Score

    style Player stroke-width:4px, fill:lightblue
    style Rocket fill:lightblue
    style Cannon stroke-width:4px, fill:red
    style Cannon-Ball fill:red
    style City stroke-width:4px, fill:lightblue

```