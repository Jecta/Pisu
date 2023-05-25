import color from "chalk";
import readline from "readline";
import { base64encode, base64decode } from "nodejs-base64"

let rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

function question(string) {
    return new Promise((res) => {
        rl.question(string, (answer) => res(answer))
    })
}

async function Pisu() {

    console.clear();

    console.log("\n")

    console.log(color.red("\n" +
        " ██▓███   ██▓  ██████  █    ██ \n" +
        "▓██░  ██▒▓██▒▒██    ▒  ██  ▓██▒\n" +
        "▓██░ ██▓▒▒██▒░ ▓██▄   ▓██  ▒██░\n" +
        "▒██▄█▓▒ ▒░██░  ▒   ██▒▓▓█  ░██░\n" +
        "▒██▒ ░  ░░██░▒██████▒▒▒▒█████▓ \n" +
        "▒▓▒░ ░  ░░▓  ▒ ▒▓▒ ▒ ░░▒▓▒ ▒ ▒ \n" +
        "░▒ ░      ▒ ░░ ░▒  ░ ░░░▒░ ░ ░ \n" +
        "░░        ▒ ░░  ░  ░   ░░░ ░ ░ \n" +
        "          ░        ░     ░     \n" +
        "                               \n"));

    question(color.blue('Please enter the userID: ')).then(async Results => {
        await Results

        if(isNaN(Results)) {
            return console.log(color.white("There are only numbers in a userID."))
        } else {

            let tokenpart = base64encode(Results.toString());

            console.log(color.red("First Token Part: " + tokenpart));

        }

    })
}
 Pisu()