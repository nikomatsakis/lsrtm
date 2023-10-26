package auth;

public final class HttpAuth {

    public Authenticated authenticate(HttpRequest request) throws AuthenticationException {
        if (request.path().equals("/foo")) {
            return new Authenticated();
        } else {
            throw new AuthenticationExceptionUnauthenticated();
        }

    }

}