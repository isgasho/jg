#[cfg(test)]
mod cli {
    use std::process::Command;

    use assert_cmd::prelude::*;

    #[test]
    fn should_match_array_with_anything_in_it() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("[.]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "[{\"name\":\"inigo montoya\"}]\n
{\"name\":\"inigo montoya\"}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("[{\"name\":\"inigo montoya\"}]\n");
    }

    #[test]
    fn should_match_array_under_prop_containing_anything_when_selecting_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".list[.]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"inigo montoya\"}\n
{\"list\":[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"list\":[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]}\n");
    }

    #[test]
    fn should_match_member_in_array_when_selecting_by_prop() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".list[.name]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"inigo montoya\",\"list\":[]}\n
{\"list\":[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"list\":[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]}\n");
    }

    #[test]
    fn should_match_sequence_matchers_within_other_sequences() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".bid_request.imp[0].pmp.deals[{"id":"BIDDER-DEAL-1"}]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"bid_request\":{\"imp\":[{\"pmp\":{\"deals\":[{\"id\":\"BIDDER-DEAL-1\"}],\"private_auction\":0}}]}}\n");

        assert_cmd.assert().success().stdout("{\"bid_request\":{\"imp\":[{\"pmp\":{\"deals\":[{\"id\":\"BIDDER-DEAL-1\"}],\"private_auction\":0}}]}}\n");
    }

    #[test]
    fn should_not_match_empty_array_when_selecting_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".list[.]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"inigo montoya\"}\n
{\"list\":[]}\n",
        );

        assert_cmd.assert().failure();
    }
}