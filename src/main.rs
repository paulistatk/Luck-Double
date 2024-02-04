use std::env;
use std::fs;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Por favor, forneça um número como argumento.");
        return;
    }

    let num_words = match args[1].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            println!("Por favor, forneça um número válido como argumento.");
            return;
        }
    };

    let words = vec!["Acid","Aero","Agile","Alco","Alter","Answer","Ant","Arch","Art","Aspect","Audience","Aura","Aurora","Awe","Badge","Bait","Bandage","Bank","Banks","Bargain","Barrage","Basis","Bat","Beam","Beast","Beauty","Bee","Beetle","Bell","Bells","Bird","Blade","Blank","Block","Blunder","Bold","Bolt","Bomb","Bone","Bones","Bonus","Books","Boost","Boot","Boots","Brash","Brass","Brave","Bribe","Brick","Bright","Brush","Cable","Canine","Canvas","Cash","Catch","Cause","Cell","Cent","Chain","Chalk","Chance","Change","Chaos","Charge","Chart","Cheat","Checkmate","Click","Clock","Clocks","Cloud","Coarse","Coil","Collar","Complex","Console","Crash","Craven","Cross","Crumbs","Cycle","Dapper","Dare","Data","Deal","Design","Detail","Dish","Disk","Double","Dynamic","Dynamo","Edge","Ego","Elite","Eternity","Fearless","Feature","Feedback","Fickle","Fix","Flaky","Fluke","Friction","Frost","Gain","Game","Gear","Gene","Ghost","Gift","Glove","Grave","Grim","Habit","Hack","Hacks","Heat","Hide","Hollow","Hook","Ice","Impulse","Ink","Iron","Jumbo","Junior","Law","Light","Link","Lock","Luck","Mammoth","Math","Maths","Mellow","Memory","Mirror","Mouse","Nemo","Night","Nightowl","Nimble","Noise","Note","Nova","Number","Omen","Owl","Panther","Parcel","Path","Pathfinder","Patriot","Phase","Piece","Pitch","Poison","Prime","Print","Prompt","Push","Quote","Range","Rebel","Requiem","Riddle","Risk","Route","Sable","Scene","Score","Session","Shade","Shallow","Shift","Shiny","Signal","Silver","Slice","Slide","Spark","Spring","Status","Stitch","Stranger","Stretch","Survey","Switch","Thrill","Trick","Tune","Unit","Venom","Virus","Ward","Wicked","Wish","Zen","Zero","Zigzag","Zone"];
    let mut rng = rand::thread_rng();
    let mut name = String::new();

    for i in 0..num_words {
        let index = rng.gen_range(0..words.len());
        if i > 0 {
            name.push('-');
        }
        name.push_str(words[index]);
    }

    println!("Nome gerado: {}", name);

    match fs::create_dir(&name) {
        Err(why) => println!("Erro ao criar diretório: {}", why),
        Ok(_) => println!("Diretório '{}' criado com sucesso.", name),
    }
}
