import fetch from "node-fetch";
import withSession from "../../utils/withSession";
async function handler(req: any, res: any) {
  req.session.set("user", undefined);
  await req.session.save();
  res.status(200).json({
    data: "Logged out",
  });
}

export default withSession(handler);
