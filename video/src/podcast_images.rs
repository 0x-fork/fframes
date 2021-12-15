use fframes::RumotionContext;
pub use fframes::{AudioData, Frame, Video, Windows};
use svgr_macro::{self, svgr};

use crate::image;

pub fn render_frame(frame: &Frame::Frame, ctx: RumotionContext::RumotionContext) -> String {
    let me_vis = AudioData::visualize_audio_frame(
        frame,
        &AudioData::VisualizeFrameInput {
            ctx: &ctx,
            audio: ctx.get_audio_data("marketing"),
            sample_size: AudioData::SampleSize::S32,
        },
    );

    let vlad_vis = AudioData::visualize_audio_frame(
        frame,
        &AudioData::VisualizeFrameInput {
            ctx: &ctx,
            audio: ctx.get_audio_data("marketing"),
            sample_size: AudioData::SampleSize::S32,
        },
    );
    let guest_vis = AudioData::visualize_audio_frame(
        frame,
        &AudioData::VisualizeFrameInput {
            ctx: &ctx,
            audio: ctx.get_audio_data("marketing"),
            sample_size: AudioData::SampleSize::S32,
        },
    );

    svgr!(
          <svg width="1920" height="1080" viewBox="0 0 1920 1080" fill="none" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
    <g clip-path="url(#clip0)">
    <rect width="1920" height="1080" fill="#E7D850"/>
    <rect x="270" y="-318" width="1419" height="1419" fill="url(#pattern0)"/>
    <mask id="mask0" mask-type="alpha" maskUnits="userSpaceOnUse" x="-345" y="-308" width="1147" height="1561">
    <rect x="17.3406" y="-307.749" width="812.132" height="1397.76" transform="rotate(15 17.3406 -307.749)" fill="#C4C4C4"/>
    </mask>
    <g mask="url(#mask0)">
    <rect x="-344" y="-1" width="1091" height="1091" fill="url(#pattern1)"/>
    <rect opacity="0.5" x="-105" y="650" width="659" height="463" fill="url(#paint0_linear)"/>
    </g>
    <mask id="mask1" mask-type="alpha" maskUnits="userSpaceOnUse" x="1123" y="-188" width="1115" height="1519">
    <rect x="1474.91" y="-187.491" width="790.007" height="1359.68" transform="rotate(15 1474.91 -187.491)" fill="#C4C4C4"/>
    </mask>
    <g mask="url(#mask1)">
    <rect x="1134.67" y="-37.6865" width="886.181" height="1328.79" fill="url(#pattern2)"/>
    </g>
    <path d="M1285 916H1335L1245 1244H1195L1285 916Z" fill="url(#paint1_linear)"/>
    <path d="M1658 852H1708L1618 1180H1568L1658 852Z" fill="url(#paint2_linear)"/>
    <path d="M1367 834H1417L1327 1162H1277L1367 834Z" fill="url(#paint3_linear)"/>
    <path d="M1449 752H1499L1409 1080H1359L1449 752Z" fill="url(#paint4_linear)"/>
    <path d="M1486 834H1536L1446 1162H1396L1486 834Z" fill="url(#paint5_linear)"/>
    <path d="M1556 796H1606L1516 1124H1466L1556 796Z" fill="url(#paint6_linear)"/>
    <path d="M1744 752H1794L1704 1080H1654L1744 752Z" fill="url(#paint7_linear)"/>
    <path d="M1781 834H1831L1741 1162H1691L1781 834Z" fill="url(#paint8_linear)"/>
    <path d="M1851 796H1901L1811 1124H1761L1851 796Z" fill="url(#paint9_linear)"/>
    <path d="M1877 912H1927L1837 1240H1787L1877 912Z" fill="url(#paint10_linear)"/>
    <path d="M1914 994H1964L1874 1322H1824L1914 994Z" fill="url(#paint11_linear)"/>
    <path d="M1984 956H2034L1944 1284H1894L1984 956Z" fill="url(#paint12_linear)"/>
    <path d="M1576 934H1626L1536 1262H1486L1576 934Z" fill="url(#paint13_linear)"/>
    <path d="M1271 752H1321L1231 1080H1181L1271 752Z" fill="url(#paint14_linear)"/>
    <path d="M100 869H150L60 1197H10L100 869Z" fill="url(#paint15_linear)"/>
    <path d="M-2 813H48L-42 1141H-92L-2 813Z" fill="url(#paint16_linear)"/>
    <path d="M186 769H236L146 1097H96L186 769Z" fill="url(#paint17_linear)"/>
    <path d="M223 851H273L183 1179H133L223 851Z" fill="url(#paint18_linear)"/>
    <path d="M293 813H343L253 1141H203L293 813Z" fill="url(#paint19_linear)"/>
    <path d="M319 929H369L279 1257H229L319 929Z" fill="url(#paint20_linear)"/>
    <path d="M356 1011H406L316 1339H266L356 1011Z" fill="url(#paint21_linear)"/>
    <path d="M426 973H476L386 1301H336L426 973Z" fill="url(#paint22_linear)"/>
    <path d="M18 951H68L-22 1279H-72L18 951Z" fill="url(#paint23_linear)"/>
    <line x1="475.09" y1="1125.74" x2="793.42" y2="-62.2819" stroke="white" stroke-width="64"/>
    <line x1="1129.09" y1="1125.74" x2="1447.42" y2="-62.2819" stroke="white" stroke-width="64"/>
    </g>
    <defs>
    <pattern id="pattern0" patternContentUnits="objectBoundingBox" width="1" height="1">
    <use xlink:href="#image0" transform="scale(0.0015625)"/>
    </pattern>
    <pattern id="pattern1" patternContentUnits="objectBoundingBox" width="1" height="1">
    <use xlink:href="#image1" transform="scale(0.0015625)"/>
    </pattern>
    <pattern id="pattern2" patternContentUnits="objectBoundingBox" width="1" height="1">
    <use xlink:href="#image2" transform="translate(0 -0.000121993) scale(0.000366166 0.0002442)"/>
    </pattern>
    <linearGradient id="paint0_linear" x1="224.5" y1="650" x2="224.5" y2="1113" gradientUnits="userSpaceOnUse">
    <stop stop-opacity="0"/>
    <stop offset="1"/>
    </linearGradient>
    <linearGradient id="paint1_linear" x1="1662.88" y1="916" x2="1662.88" y2="1244" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint2_linear" x1="2035.88" y1="852" x2="2035.88" y2="1180" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint3_linear" x1="1744.88" y1="834" x2="1744.88" y2="1162" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint4_linear" x1="1826.88" y1="752" x2="1826.88" y2="1080" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint5_linear" x1="1863.88" y1="834" x2="1863.88" y2="1162" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint6_linear" x1="1933.88" y1="796" x2="1933.88" y2="1124" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint7_linear" x1="2121.88" y1="752" x2="2121.88" y2="1080" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint8_linear" x1="2158.88" y1="834" x2="2158.88" y2="1162" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint9_linear" x1="2228.88" y1="796" x2="2228.88" y2="1124" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint10_linear" x1="2254.88" y1="912" x2="2254.88" y2="1240" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint11_linear" x1="2291.88" y1="994" x2="2291.88" y2="1322" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint12_linear" x1="2361.88" y1="956" x2="2361.88" y2="1284" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint13_linear" x1="1953.88" y1="934" x2="1953.88" y2="1262" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint14_linear" x1="1648.88" y1="752" x2="1648.88" y2="1080" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint15_linear" x1="477.878" y1="869" x2="477.878" y2="1197" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint16_linear" x1="375.878" y1="813" x2="375.878" y2="1141" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint17_linear" x1="563.878" y1="769" x2="563.878" y2="1097" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint18_linear" x1="600.878" y1="851" x2="600.878" y2="1179" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint19_linear" x1="670.878" y1="813" x2="670.878" y2="1141" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint20_linear" x1="696.878" y1="929" x2="696.878" y2="1257" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint21_linear" x1="733.878" y1="1011" x2="733.878" y2="1339" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint22_linear" x1="803.878" y1="973" x2="803.878" y2="1301" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <linearGradient id="paint23_linear" x1="395.878" y1="951" x2="395.878" y2="1279" gradientUnits="userSpaceOnUse">
    <stop stop-color="#E7D850" stop-opacity="0"/>
    <stop offset="1" stop-color="#E7D850"/>
    </linearGradient>
    <clipPath id="clip0">
    <rect width="1920" height="1080" fill="white"/>
    </clipPath>
    <image id="image0" width="640" height="640" xlink:href={image::getMe()}/>
    <image id="image1" width="640" height="640" xlink:href={image::getGuest()}/>
    <image id="image2" width="2731" height="4096" xlink:href={image::getVlad()}/>
    </defs>
    </svg>

        )
}
