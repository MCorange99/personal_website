import monero_qr from "../assets/support/monero.png";

export default function Home() {
    return (
        <>  
            <h1>Consider supporting my work!</h1>
            

            <h2>My ko-fi is down below in the left corner</h2>
            <h2>My monero address:</h2>
            <p>46nvfTRV4HB1n3mTjWGFfKAeyu4h7mpsA96pKfJyZkyrcHm9q99fGogiTvynTUxsgFQABiCEffDGUTKBRiSTHR36KmTQ8aH</p>
            <img src={monero_qr}></img>

            <h1>Thank you, for supporting my work &lt;3</h1>


            {//@ts-ignore
            kofiWidgetOverlay.draw('mcorange', {
                'type': 'floating-chat',
                'floating-chat.donateButton.text': 'Support me',
                'floating-chat.donateButton.background-color': '#fcbf47',
                'floating-chat.donateButton.text-color': '#323842'
            })}

        </>
    )
}