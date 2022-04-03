// import fetch from 'node-fetch';
import fetch from "../../utils/fetch";
import withSession from "../../utils/withSession";

async function handler(req: any, res: any) {
  try {
    const user = req.session.get("user");
    let r = await fetch("/images", user, req.body, "POST");
    res.status(200).json(await r.json());
  } catch (e) {
    console.log(e);
  }
}

export default withSession(handler);
