let score (dice: List<int>) =
    let counts = dice |> List.groupBy id |> List.map (fun (k, v) -> (k, v.Length))
    counts |> List.sumBy (fun (k, v) -> 
        match k with
        | 1 when v >= 3 -> 1000 + (v - 3) * 100
        | 1 -> v * 100
        | 5 when v >= 3 -> 500 + (v - 3) * 50
        | 5 -> v * 50
        | k when v >= 3 -> k * 100
        | _ -> 0)

let test1 = score [1; 1; 1; 5; 1] // 1150
let test2 = score [2; 3; 4; 6; 2] // 0
let test3 = score [3; 4; 5; 3; 3] // 350

printfn "%A" test1
printfn "%A" test2
printfn "%A" test3
