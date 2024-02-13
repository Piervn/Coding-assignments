let comp(a, b): bool =
    (b |> List.sort) = (a |> List.map (fun x -> x*x) |> List.sort)

let a1 = [121; 144; 19; 161; 19; 144; 19; 11]
let b1 = [11*11; 121*121; 144*144; 19*19; 161*161; 19*19; 144*144; 19*19]
let res1 = comp(a1, b1)
printfn "%A" res1

let a2 = [121; 144; 19; 161; 19; 144; 19; 11]
let b2 = [11*21; 121*121; 144*144; 19*19; 161*161; 19*19; 144*144; 19*19]
let res2 = comp(a2, b2)
printfn "%A" res2

