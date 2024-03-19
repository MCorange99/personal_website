import { Route, Router, Routes, useParams } from "@solidjs/router";

const redirects = {
    "github": "https://github.com/MCorange99",
    "ko-fi": "https://ko-fi.com/mcorange",
    "mastadon": "https://bark.lgbt/@mcorange",
};


export default () => {
    const params = useParams(); 
    //@ts-ignore
    const url = (redirects[params["id"] as string] || "/") as string;
    redirect(url);

    return (
        <h1>Redirecting...</h1>
    )

}






function redirect(p: string) {
    window.location.assign(p);
}
