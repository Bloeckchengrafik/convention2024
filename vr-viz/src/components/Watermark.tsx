function WatermarkSvg() {
    return <svg width="42" height="42" viewBox="4 4 43 42" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g clip-path="url(#clip0_14_71)">
            <g clip-path="url(#clip1_14_71)">
                <mask id="mask0_14_71" style={{maskType: "alpha"}} maskUnits="userSpaceOnUse" x="-8" y="4" width="68"
                      height="70">
                    <path fill-rule="evenodd" clip-rule="evenodd"
                          d="M10.2146 19C9.6623 19 9.21458 19.4477 9.21458 20V30.5205C9.21458 31.0728 9.6623 31.5205 10.2146 31.5205H17.5579C17.8776 31.5205 18.1781 31.3676 18.3663 31.1092L21.7666 26.4401C23.3641 24.2464 26.6359 24.2464 28.2335 26.4401L31.6337 31.1092C31.8219 31.3676 32.1224 31.5205 32.4421 31.5205H40.7854C41.3377 31.5205 41.7854 31.0728 41.7854 30.5205V20C41.7854 19.4477 41.3377 19 40.7854 19H10.2146ZM35.5745 36.5206C35.5745 36.5205 35.5745 36.5205 35.5745 36.5205H42.7854C44.9946 36.5205 46.7854 34.7296 46.7854 32.5205V18C46.7854 15.7909 44.9946 14 42.7854 14H8.21458C6.00545 14 4.21458 15.7909 4.21458 18V32.5205C4.21458 34.7296 6.00545 36.5205 8.21459 36.5205H14.4255C14.4255 36.5205 14.4255 36.5205 14.4255 36.5206V36.5206C14.4255 36.5206 14.4255 36.5206 14.4255 36.5206H20.102C20.4217 36.5206 20.7221 36.3678 20.9103 36.1093L24.1916 31.6035C24.591 31.0551 25.409 31.0551 25.8084 31.6035L29.0897 36.1093C29.2779 36.3678 29.5783 36.5206 29.898 36.5206H35.5745C35.5745 36.5206 35.5745 36.5206 35.5745 36.5206V36.5206Z"
                          fill="#7390AA"/>
                </mask>
                <g mask="url(#mask0_14_71)">
                    <rect width="25" height="50" fill="#FF5F5F"/>
                    <rect x="25" width="25" height="50" fill="#5FB2FF"/>
                </g>
            </g>
        </g>
        <defs>
            <clipPath id="clip0_14_71">
                <rect width="50" height="50" fill="white"/>
            </clipPath>
            <clipPath id="clip1_14_71">
                <rect width="43" height="23" fill="white" transform="translate(4 14)"/>
            </clipPath>
        </defs>
    </svg>

}

export function Watermark() {
    return <div className="watermark">
        <WatermarkSvg/>
        <div>&nbsp;</div>
        <div>&nbsp;</div>
        <span>ftVR Wizard</span>
    </div>
}