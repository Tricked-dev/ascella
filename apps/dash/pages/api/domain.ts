// import fetch from 'node-fetch';
import withSession from '../../utils/withSession';
import fetch from '../../utils/fetch';

async function handler(req: any, res: any) {
	try {
		// console.log(req.body);
		let json = req.body;
		const user = req.session.get('user');
		let domain = json.domain;

		let r = await fetch(
			'/domain',
			user,
			JSON.stringify({
				domain: domain,
			}),
			'POST'
		);

		if (r.ok) {
			req.session.set('user', {
				...(user as any),
				domain: domain,
			});
			await req.session.save();
			res.status(200).json({
				data: 'Ok',
			});
		} else {
			res.status(400).json({
				data: 'Error',
				...((await r.json()) as any),
			});
		}
	} catch (e) {
		console.log(e);
	}
}

export default withSession(handler);
