import fetch from "../../utils/fetch";
import withSession from "../../utils/withSession";
async function handler(req: any, res: any) {
    let user = await fetch(
        "/verify",
        {
            id: req.body.id,
            key: req.body.key,
        },
        JSON.stringify({
            id: req.body.id,
            key: req.body.key,
        }),
        "POST",
    );
    if (user.ok) {
        req.session.set("user", {
            ...((await user.json()) as any),
            id: req.body.id,
            key: req.body.key,
        });
        await req.session.save();
        res.status(200).json({
            data: "valid user",
        });
    } else {
        res.status(400).json({
            data: "invalid user",
        });
    }
}

export default withSession(handler);
