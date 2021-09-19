pub struct SpringRuntime {
    pub m_zeta: f32,
    pub w0: f32,
    pub wd: f32,
    pub a: f32,
    pub b: f32,
}

pub struct SpringOptions {
    pub mass: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl SpringRuntime {
    pub fn from_options(opts: &SpringOptions) -> SpringRuntime {
        let m_zeta = opts.damping / (2.0 * libm::sqrtf(opts.stiffness * opts.mass));
        let m_w0 = libm::sqrtf(opts.stiffness / opts.mass);

        if m_zeta < 1.0 {
            let m_wd = m_w0 * libm::sqrtf(1.0 - m_zeta * m_zeta);
            // Under-damped
            SpringRuntime {
                m_zeta,
                w0: m_w0,
                wd: m_wd,
                a: 1.0,
                b: (m_zeta * m_w0) / m_wd,
            }
        } else {
            // Critically damped (ignoring over-damped case for now).
            SpringRuntime {
                m_zeta,
                w0: m_w0,
                wd: 0.0,
                a: 1.0,
                b: m_w0,
            }
        }
    }
}
