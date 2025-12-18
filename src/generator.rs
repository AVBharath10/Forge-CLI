
#[derive(Debug, Clone)]
pub enum Framework {
    NextJs,
    Express,
}

#[derive(Debug, Clone)]
pub enum Database {
    Postgres,
}

#[derive(Debug, Clone)]
pub enum Auth {
    None,
    Jwt,
}

pub fn generate_env(framework: &Framework, auth: &Auth) -> String {
    let mut env = String::new();

    // Shared DB
    env.push_str(
        "DATABASE_URL=\"postgresql://appuser:applocal@localhost:5432/appdb\"\n",
    );

    match framework {
        Framework::NextJs => {
            env.push_str("NEXT_PUBLIC_API_URL=\"http://localhost:3001\"\n");
            env.push_str("NEXT_PUBLIC_APP_NAME=\"forge-app\"\n");
        }
        Framework::Express => {
            env.push_str("PORT=3001\n");
            env.push_str("NODE_ENV=development\n");
        }
    }

    match auth {
        Auth::None => {}
        Auth::Jwt => {
            env.push_str("JWT_SECRET=\"change-me-super-secret-key\"\n");
            if let Framework::NextJs = framework {
                env.push_str("NEXTAUTH_URL=\"http://localhost:3000\"\n");
            }
        }
    }

    env
}
