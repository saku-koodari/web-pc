const fs = require("fs");
const csv = require("csv-parser");

const results = [];

const outToEnum = (out) => {
  return out
    .toString()
    .toUpperCase()
    .replace("0", "_Zero_")
    .replace("1", "_One_")
    .replace("-", "_Minus_")
    .replace("!", "_Neg_")
    .replace("+", "_Plus_")
    .replace("&", "_And_")
    .replace("|", "_Or_")
    .replace("_", "")
    .replace("_", "")
    .replace("_", "")
    .replace("_", "");
};

fs.createReadStream("codes.csv")
  .pipe(csv())
  .on("data", (data) => results.push(data))
  .on("end", () => {
    console.log("pub enum Opcode {");
    const opcodes = results.map((r) => {
      console.log("  " + outToEnum(r.out) + ",");
    });
    console.log("}");
    console.log("");

    const parsed = results.map((r) => {
      const nr = r.nr;
      const zx = r.zx;
      const nx = r.nx;
      const zy = r.zy;
      const ny = r.ny;
      const f = r.f;
      const no = r.no;
      const out = r.out;
      /////////////////
      const number_int = +nr + +zx + +nx + +zy + +ny + +f + +no;
      const int = number_int.toString();
      const bin = number_int.toString(2);
      const hex = "0x" + number_int.toString(4);

      console.log(
        `opcodes.insert(Opcode::${outToEnum(
          out
        )}, AluControlBits {zx:${zx},nx:${nx},zy:${zy},ny:${ny},f:${f},no:${no},` +
          `int:String::from("${int}"), hex:String::from("${hex}"), bin:String::from("${bin}")});`
      );

      return {
        nr,
        zx,
        nx,
        zy,
        ny,
        f,
        no,
        out,
        int,
        bin,
        hex,
      };
    });

    console.log("");
  });

console.log("");
