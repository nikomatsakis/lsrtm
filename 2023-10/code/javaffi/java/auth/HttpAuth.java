package auth;

public final class HttpAuth {

    public Authenticated authenticate(HttpRequest request) throws AuthenticationException {
        if (request.path() == "/foo") {
            return new Authenticated();
        } else {
            throw new AuthenticationExceptionUnauthenticated();
        }

    }

}