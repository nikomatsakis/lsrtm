
package auth;

import java.util.List;
import java.util.Map;

public record HttpRequest(
        String verb,
        String path) {
}
