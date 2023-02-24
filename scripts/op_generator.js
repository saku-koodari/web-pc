// Script to generate Rust code
const fs = require("fs");
const csv = require("csv-parser");
const Handlebars = require("handlebars");

const opcodes_rust_template_variables = {
  now: new Date().toISOString(),
  templateFileName: "opcodes.handlebars",
  csvDataFileName: "codes.csv",

  opcodes_enum: [],
  opcodes_hasmap_inserts: [],
};

const opcodes_rust_template_str = fs.readFileSync(
  opcodes_rust_template_variables.templateFileName,
  "utf8"
);

const opcodes_rust_template = Handlebars.compile(opcodes_rust_template_str);

const csvData = [];

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

fs.createReadStream(opcodes_rust_template_variables.csvDataFileName)
  .pipe(csv())
  .on("data", (data_row) => csvData.push(data_row))
  .on("end", () => {
    csvData.forEach((row) => {
      const currentOpCodeEnum = outToEnum(row.out);
      opcodes_rust_template_variables.opcodes_enum.push(currentOpCodeEnum);

      const nr = !!row.nr;
      const zx = !!row.zx;
      const nx = !!row.nx;
      const zy = !!row.zy;
      const ny = !!row.ny;
      const f = !!row.f;
      const no = !!row.no;
      const out = row.out;

      const number_int = +nr + +zx + +nx + +zy + +ny + +f + +no;

      const name = outToEnum(out);
      const int = number_int.toString();
      const bin = number_int.toString(2);
      const hex = "0x" + number_int.toString(4);
      opcodes_rust_template_variables.opcodes_hasmap_inserts.push({
        currentOpCodeEnum,
        nr,
        zx,
        nx,
        zy,
        ny,
        f,
        no,
        name,
        out,
        int,
        bin,
        hex,
      });
    });

    const opcodes_rust_template_result = opcodes_rust_template(
      opcodes_rust_template_variables
    );
    console.log(opcodes_rust_template_result);
    fs.writeFileSync("generated/opcodes.rs", opcodes_rust_template_result);
    console.log("-- end of read codes.csv with parser --");
  });

// this code line is most likely to be excuted before csvParser ends.
