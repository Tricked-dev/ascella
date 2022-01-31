import isoFetch from 'isomorphic-fetch';

export default function fetch(
	route: string,
	session: any,
	body?: any,
	method?: string
): Promise<Response> {
	return isoFetch('https://ascella.wtf/v2/ascella' + route, {
		// return isoFetch('http://localhost:7878/v2/ascella' + route, {
		method,
		body,
		headers: {
			'x-user-id': session.id,
			'x-user-token': session.key,
			'user-agent': 'Ascella Dashboard (dash.ascella.host)',
		},
	});
}
