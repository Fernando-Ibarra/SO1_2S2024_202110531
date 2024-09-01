console.log("Hi! From main.js on Docker container with Image 2")

const timeOutFor = async() => {
    await setTimeout(() => {
        for (let i = 0; i < 1000; i++) { 
            console.log(i)
        }
    }, 1000);
}

// random number between 250 a 500
const randomNum2 = Math.floor(Math.random() * (500 - 250) + 250);

const main = () => {
    setInterval(async() => {
        console.log("Container is running...");
        await timeOutFor();
    }, randomNum2);
}

main();