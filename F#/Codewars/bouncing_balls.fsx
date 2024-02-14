let rec bouncingBall (h: float) (bounce: float) (window: float) =
    if h <= 0. || bounce <= 0. || bounce >= 1. || window >= h then -1
    else 2 + bouncingBall (h * bounce) bounce window
    
printfn "%A" (3 = bouncingBall 3. 0.66 1.5)
printfn "%A" (15 = bouncingBall 30.0 0.66 1.5)
printfn "%A" (21 = bouncingBall 30.0 0.75 1.5)
printfn "%A" (7 = bouncingBall 330.0 0.4 10.0)
printfn "%A" (3 = bouncingBall 40.0 0.4 10.0)