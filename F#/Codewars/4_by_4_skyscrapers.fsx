let rec permutations list: int list list =
    match list with
    | [] -> [[]]
    | xs -> [ for x in xs do for ys in permutations (List.filter ((<>) x) xs) -> x :: ys ]

let perms = permutations [1; 2; 3; 4]

let getVisibleSkyscrapers row = 
    row |> List.fold (fun acc e ->
        if e > (fst acc) then
            (e, (snd acc) + 1)
        else
            acc
    ) (0, 0) |> snd

let permsWithClues = 
    perms |> List.map (fun perm -> 
    (
        getVisibleSkyscrapers perm,
        perm,
        getVisibleSkyscrapers (perm |> List.rev)
    ))

let solvePuzzle (clues : int[]) =
    let chunked = clues |> Array.chunkBySize 4
    let rows = Array.zip (chunked.[3] |> Array.rev) chunked.[1]
    let columns = Array.zip chunked.[0] (chunked.[2] |> Array.rev)
    let rec getAllPossibleSolutions i (columns: int list list) =
        match i with
        | 4 -> [[]]
        | _ -> [ 
            for (l, perm, r) in
                permsWithClues |> List.filter (fun (l, perm, r) -> 
                let leftClue = fst rows[i]
                let rightClue = snd rows[i]
                (l = leftClue || leftClue = 0) && (r = rightClue || rightClue = 0) && 
                (not <| ((perm, columns) ||> List.mapi2 (fun key e column -> columns[key] |> List.contains e) |> List.contains true))) 
                do
                    let newColumns = (columns, perm) ||> List.map2 (fun column e -> e :: column)
                    for rest in getAllPossibleSolutions (i+1) newColumns -> perm :: rest
            ]
    let possibleSolutions = getAllPossibleSolutions 0 (List.init 4 (fun _ -> []))
    possibleSolutions |> List.filter (fun solution -> 
        solution |> List.transpose |> List.mapi (fun i column -> 
            let topSkyscrapers = getVisibleSkyscrapers column
            let bottomSkyscrapers = getVisibleSkyscrapers (column |> List.rev)
            let (topClue, bottomClue) = columns[i]
            (topClue = 0 || topClue = topSkyscrapers) && (bottomClue = 0 || bottomClue = bottomSkyscrapers)
        ) |> List.contains false |> not
    ) |> List.item 0 |> List.map Array.ofList |> Array.ofList



let clues1 = [| 2; 2; 1; 3; 2; 2; 3; 1; 1; 2; 2; 3; 3; 2; 1; 3; |]
let clues2 = [| 0; 0; 1; 2; 0; 2; 0; 0; 0; 3; 0; 0; 0; 1; 0; 0; |]
let solution1 = solvePuzzle clues1
let solution2 = solvePuzzle clues2

solution1 |> printfn "%A"
solution2 |> printfn "%A"