import * as bcrypt from 'bcrypt';

const args = process.argv.slice(2);

if (args[0]) {
    const salt = bcrypt.genSaltSync(10);
    const hash = bcrypt.hashSync(args[0], salt);
    console.log("hashed pwd:", hash);
}

