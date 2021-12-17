import { join } from 'path';
import { readFile, writeFile } from 'fs/promises';
const file = join('prisma', 'schema.prisma');

let renames = {
	String: 'String',
	Int: 'i32',
	DateTime: 'String',
	Boolean: 'Bool',
};

const main = async () => {
	let data = (await readFile(file)).toString('utf8');
	let structs = data
		.split('model')
		.slice(1)
		.map((x) => {
			let name = x.trim().split(' ')[0];
			let keys = x
				.trim()
				.split('\n')
				.slice(1)
				.slice(0, -1)
				.map((x) => x.trim().split(/\s+/).join(' '))
				.map((x) => {
					if (x.includes('}') || !x) return;

					let optional = false;
					let name = x.split(' ')[0];
					let data = x.split(' ')[1];
					if (data.endsWith('?')) {
						optional = true;
						data = data.slice(0, -1);
					}
					//@ts-ignore -
					data = renames[data as keyof renames] || data;
					return `pub ${name}: ${optional ? `Option<${data}>` : data}`;
				});
			return `#[derive(PostgresMapper,Serialize, Deserialize, Clone)]\n#[pg_mapper(table = "${name}")]\npub struct ${name} {\n${keys.join(
				',\n'
			)}\n}`;
		});
	writeFile(
		'struct.rs',
		Buffer.from(
			`use serde::{Deserialize, Serialize};\nuse tokio_pg_mapper_derive::PostgresMapper;\n${structs.join(
				'\n\n'
			)}`
		)
	);
	console.log(structs.join('\n\n'));
};
main();
