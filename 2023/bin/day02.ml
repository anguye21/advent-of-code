let max_reds = 12
let max_greens = 13
let max_blues = 14

let is_draw_possible draw =
  let open Str in
  let _ = string_match (regexp {|.* \([0-9]+\) blue.*|}) draw 0 in
  let num_blues =
    int_of_string
      (try matched_group 1 draw with
       | Invalid_argument _ -> "0")
  in
  let blue_possible = num_blues <= max_blues in
  let _ = string_match (regexp {|.* \([0-9]+\) red.*|}) draw 0 in
  let num_reds =
    int_of_string
      (try matched_group 1 draw with
       | Invalid_argument _ -> "0")
  in
  let red_possible = num_reds <= max_reds in
  let _ = string_match (regexp {|.* \([0-9]+\) green.*|}) draw 0 in
  let num_greens =
    int_of_string
      (try matched_group 1 draw with
       | Invalid_argument _ -> "0")
  in
  let green_possible = num_greens <= max_greens in
  blue_possible && red_possible && green_possible
;;

let is_game_possible game =
  let draws = String.split_on_char ';' game |> List.map String.trim in
  List.fold_left
    (fun acc draw -> acc && is_draw_possible (" " ^ draw))
    true
    draws
;;

let rec part1_helper lines id =
  match lines with
  | [] -> 0
  | h :: t ->
    let game =
      String.split_on_char ':' h |> List.tl |> List.hd |> String.trim
    in
    let inc = if is_game_possible game then id else 0 in
    inc + part1_helper t (id + 1)
;;

let part1 lines = part1_helper lines 1

type draw =
  { blues : int
  ; reds : int
  ; greens : int
  }

let draw_of_string draw =
  let open Str in
  let s = " " ^ draw in
  let _ = string_match (regexp {|.* \([0-9]+\) blue.*|}) s 0 in
  let num_blues =
    int_of_string
      (try matched_group 1 s with
       | Invalid_argument _ -> "0")
  in
  let _ = string_match (regexp {|.* \([0-9]+\) red.*|}) s 0 in
  let num_reds =
    int_of_string
      (try matched_group 1 s with
       | Invalid_argument _ -> "0")
  in
  let _ = string_match (regexp {|.* \([0-9]+\) green.*|}) s 0 in
  let num_greens =
    int_of_string
      (try matched_group 1 s with
       | Invalid_argument _ -> "0")
  in
  { blues = num_blues; reds = num_reds; greens = num_greens }
;;

let cube_set_power game =
  let draws = String.split_on_char ';' game |> List.map String.trim in
  let parsed_draws = List.map draw_of_string draws in
  let min_blues =
    List.fold_left
      (fun acc draw -> if acc > draw.blues then acc else draw.blues)
      0
      parsed_draws
  in
  let min_reds =
    List.fold_left
      (fun acc draw -> if acc > draw.reds then acc else draw.reds)
      0
      parsed_draws
  in
  let min_greens =
    List.fold_left
      (fun acc draw -> if acc > draw.greens then acc else draw.greens)
      0
      parsed_draws
  in
  min_blues * min_reds * min_greens
;;

let rec part2 lines =
  match lines with
  | [] -> 0
  | h :: t ->
    let game =
      String.split_on_char ':' h |> List.tl |> List.hd |> String.trim
    in
    let power = cube_set_power game in
    power + part2 t
;;

let () =
  let lines = Aoc.read_lines "./input/day02.prod.input" in
  Printf.printf "part1: %d\n" (part1 lines);
  Printf.printf "part2: %d\n" (part2 lines)
;;
