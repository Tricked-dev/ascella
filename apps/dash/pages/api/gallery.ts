// import fetch from 'node-fetch';
import fetch from '../../utils/fetch';
import withSession from '../../utils/withSession';

async function handler(req: any, res: any) {
	try {
		let json = req.body;
		const user = req.session.get('user');
		let skip = json.skip;

		let r = await fetch(
			'/images',
			user,
			JSON.stringify({
				skip: skip || 0,
			}),
			'POST'
		);
		console.log(r);

		// console.log(r.json());
		res.status(200).json(await r.json());
	} catch (e) {
		console.log(e);
	}
}

export default withSession(handler);
