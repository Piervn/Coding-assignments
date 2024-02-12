let rec gcd a b =
    match a, b with
    | a, 0 -> a
    | a, b -> gcd b (a % b)

let lcm a b = a * b / gcd a b

let convertFracts (ls: (int * int) list) =
    match ls with
    | [] -> []
    | [x] -> [x]
    | xs ->
        let denom = xs |> List.fold (fun acc (a, b) -> lcm acc (b / gcd a b)) 1
        xs |> List.map (fun (a, b) -> (a * denom / b, denom))

let test1 = [(69, 130); (87, 1310); (3, 4)]
let test2 = [(690, 1300); (87, 1310); (30, 40)]

let res1 = convertFracts test1
let res2 = convertFracts test2

printfn "%A" res1
printfn "%A" res2
printfn "%A" (res1 = res2)