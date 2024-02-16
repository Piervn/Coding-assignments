let productFib (n: uint64) =
    let rec fib a b =
        match (a * b) with
        | x when x > n -> (a, b, false)
        | x when x = n -> (a, b, true)
        | _ -> fib b (a + b)

    fib 0UL 1UL        

let test1 = productFib 4895UL
let test2 = productFib 5895UL
let test3 = productFib 74049690UL

printfn "%A" (test1 = (55UL, 89UL, true))
printfn "%A" (test2 = (89UL, 144UL, false))
printfn "%A" (test3 = (6765UL, 10946UL, true))
