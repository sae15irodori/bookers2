
Bookモデルのテスト
  バリデーションのテスト
    titleカラム
[32m      空欄でないこと[0m
    bodyカラム
[32m      空欄でないこと[0m
[32m      200文字以下であること: 200文字は〇[0m
[32m      200文字以下であること: 201文字は×[0m
  アソシエーションのテスト
    Userモデルとの関係
[32m      N:1となっている[0m

Userモデルのテスト
  バリデーションのテスト
    nameカラム
[32m      空欄でないこと[0m
[32m      2文字以上であること: 1文字は×[0m
[32m      2文字以上であること: 2文字は〇[0m
[32m      20文字以下であること: 20文字は〇[0m
[32m      20文字以下であること: 21文字は×[0m
[32m      一意性があること[0m
    introductionカラム
[32m      50文字以下であること: 50文字は〇[0m
[32m      50文字以下であること: 51文字は×[0m
  アソシエーションのテスト
    Bookモデルとの関係
[32m      1:Nとなっている[0m

[STEP1] ユーザログイン前のテスト
  トップ画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
[32m      Log inリンクが表示される: 青色のボタンの表示が「Log in」である[0m
[32m      Log inリンクの内容が正しい[0m
[32m      Sign upリンクが表示される: 緑色のボタンの表示が「Sign up」である[0m
[32m      Sign upリンクの内容が正しい[0m
  アバウト画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
  ヘッダーのテスト: ログインしていない場合
    表示内容の確認
[32m      Bookersリンクが表示される: 左上から1番目のリンクが「Bookers」である[0m
[32m      Homeリンクが表示される: 左上から2番目のリンクが「Home」である[0m
[32m      Aboutリンクが表示される: 左上から3番目のリンクが「About」である[0m
[32m      Sign upリンクが表示される: 左上から4番目のリンクが「Sign up」である[0m
[32m      Log inリンクが表示される: 左上から5番目のリンクが「Log in」である[0m
    リンクの内容を確認
[32m      Bookersを押すと、トップ画面に遷移する[0m
[32m      Homeを押すと、トップ画面に遷移する[0m
[32m      Aboutを押すと、アバウト画面に遷移する[0m
[32m      Sign upを押すと、新規登録画面に遷移する[0m
[32m      Log inを押すと、ログイン画面に遷移する[0m
  ユーザ新規登録のテスト
    表示内容の確認
[32m      URLが正しい[0m
[32m      「Sign up」と表示される[0m
[32m      nameフォームが表示される[0m
[32m      emailフォームが表示される[0m
[32m      passwordフォームが表示される[0m
[32m      password_confirmationフォームが表示される[0m
[32m      Sign upボタンが表示される[0m
    新規登録成功のテスト
[32m      正しく新規登録される[0m
[32m      新規登録後のリダイレクト先が、新規登録できたユーザの詳細画面になっている[0m
  ユーザログイン
    表示内容の確認
[32m      URLが正しい[0m
[32m      「Log in」と表示される[0m
[32m      nameフォームが表示される[0m
[32m      passwordフォームが表示される[0m
[32m      Log inボタンが表示される[0m
[32m      emailフォームは表示されない[0m
    ログイン成功のテスト
[32m      ログイン後のリダイレクト先が、ログインしたユーザの詳細画面になっている[0m
    ログイン失敗のテスト
[32m      ログインに失敗し、ログイン画面にリダイレクトされる[0m
  ヘッダーのテスト: ログインしている場合
    ヘッダーの表示を確認
[32m      Bookersリンクが表示される: 左上から1番目のリンクが「Bookers」である[0m
[32m      Homeリンクが表示される: 左上から2番目のリンクが「Home」である[0m
[32m      Usersリンクが表示される: 左上から3番目のリンクが「Users」である[0m
[32m      Booksリンクが表示される: 左上から4番目のリンクが「Books」である[0m
[32m      Log outリンクが表示される: 左上から5番目のリンクが「Log out」である[0m
  ユーザログアウトのテスト
    ログアウト機能のテスト
[32m      正しくログアウトできている: ログアウト後のリダイレクト先においてAbout画面へのリンクが存在する[0m
[32m      ログアウト後のリダイレクト先が、トップになっている[0m

[STEP2] ユーザログイン後のテスト
  ヘッダーのテスト: ログインしている場合
    リンクの内容を確認: ※logoutは『ユーザログアウトのテスト』でテスト済みになります。
[32m      Homeを押すと、自分のユーザ詳細画面に遷移する[0m
[32m      Usersを押すと、ユーザ一覧画面に遷移する[0m
[32m      Booksを押すと、投稿一覧画面に遷移する[0m
  投稿一覧画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
[32m      自分と他人の画像のリンク先が正しい[0m
[32m      自分の投稿と他人の投稿のタイトルのリンク先がそれぞれ正しい[0m
[32m      自分の投稿と他人の投稿のオピニオンが表示される[0m
    サイドバーの確認
[32m      自分の名前と紹介文が表示される[0m
[32m      自分のユーザ編集画面へのリンクが存在する[0m
[32m      「New book」と表示される[0m
[32m      titleフォームが表示される[0m
[32m      titleフォームに値が入っていない[0m
[32m      bodyフォームが表示される[0m
[32m      bodyフォームに値が入っていない[0m
[32m      Create Bookボタンが表示される[0m
    投稿成功のテスト
[31m      自分の新しい投稿が正しく保存される (FAILED - 1)[0m
[31m      リダイレクト先が、保存できた投稿の詳細画面になっている (FAILED - 2)[0m
  自分の投稿詳細画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
[32m      「Book detail」と表示される[0m
[32m      ユーザ画像・名前のリンク先が正しい[0m
[32m      投稿のtitleが表示される[0m
[32m      投稿のbodyが表示される[0m
[32m      投稿の編集リンクが表示される[0m
[32m      投稿の削除リンクが表示される[0m
    サイドバーの確認
[32m      自分の名前と紹介文が表示される[0m
[32m      自分のユーザ編集画面へのリンクが存在する[0m
[32m      「New book」と表示される[0m
[32m      titleフォームが表示される[0m
[32m      titleフォームに値が入っていない[0m
[32m      bodyフォームが表示される[0m
[32m      bodyフォームに値が入っていない[0m
[32m      Create Bookボタンが表示される[0m
    投稿成功のテスト
[31m      自分の新しい投稿が正しく保存される (FAILED - 3)[0m
    編集リンクのテスト
[32m      編集画面に遷移する[0m
    削除リンクのテスト
[32m      application.html.erbにjavascript_pack_tagを含んでいる[0m
[32m      正しく削除される[0m
[32m      リダイレクト先が、投稿一覧画面になっている[0m
  自分の投稿編集画面のテスト
    表示の確認
[32m      URLが正しい[0m
[32m      「Editing Book」と表示される[0m
[32m      title編集フォームが表示される[0m
[32m      body編集フォームが表示される[0m
[32m      Update Bookボタンが表示される[0m
[32m      Showリンクが表示される[0m
[32m      Backリンクが表示される[0m
    編集成功のテスト
[32m      titleが正しく更新される[0m
[32m      bodyが正しく更新される[0m
[32m      リダイレクト先が、更新した投稿の詳細画面になっている[0m
  ユーザ一覧画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
[32m      自分と他人の画像が表示される: fallbackの画像がサイドバーの1つ＋一覧(2人)の2つの計3つ存在する[0m
[32m      自分と他人の名前がそれぞれ表示される[0m
[32m      自分と他人のshowリンクがそれぞれ表示される[0m
    サイドバーの確認
[32m      自分の名前と紹介文が表示される[0m
[32m      自分のユーザ編集画面へのリンクが存在する[0m
[32m      「New book」と表示される[0m
[32m      titleフォームが表示される[0m
[32m      titleフォームに値が入っていない[0m
[32m      bodyフォームが表示される[0m
[32m      bodyフォームに値が入っていない[0m
[32m      Create Bookボタンが表示される[0m
  自分のユーザ詳細画面のテスト
    表示の確認
[32m      URLが正しい[0m
[32m      投稿一覧のユーザ画像のリンク先が正しい[0m
[32m      投稿一覧に自分の投稿のtitleが表示され、リンクが正しい[0m
[32m      投稿一覧に自分の投稿のbodyが表示される[0m
[32m      他人の投稿は表示されない[0m
    サイドバーの確認
[32m      自分の名前と紹介文が表示される[0m
[32m      自分のユーザ編集画面へのリンクが存在する[0m
[32m      「New book」と表示される[0m
[32m      titleフォームが表示される[0m
[32m      titleフォームに値が入っていない[0m
[32m      bodyフォームが表示される[0m
[32m      bodyフォームに値が入っていない[0m
[32m      Create Bookボタンが表示される[0m
  自分のユーザ情報編集画面のテスト
    表示の確認
[32m      URLが正しい[0m
[32m      名前編集フォームに自分の名前が表示される[0m
[32m      画像編集フォームが表示される[0m
[32m      自己紹介編集フォームに自分の自己紹介文が表示される[0m
[32m      Update Userボタンが表示される[0m
    更新成功のテスト
[32m      nameが正しく更新される[0m
[32m      introductionが正しく更新される[0m
[32m      リダイレクト先が、自分のユーザ詳細画面になっている[0m

[STEP3] 仕上げのテスト
  サクセスメッセージのテスト
[32m    ユーザ新規登録成功時[0m
[32m    ユーザログイン成功時[0m
[32m    ユーザログアウト成功時[0m
[32m    ユーザのプロフィール情報更新成功時[0m
[31m    投稿データの新規投稿成功時: 投稿一覧画面から行います。 (FAILED - 4)[0m
[32m    投稿データの更新成功時[0m
  処理失敗時のテスト
    ユーザ新規登録失敗: nameを1文字にする
[32m      新規登録されない[0m
[32m      新規登録画面を表示しており、フォームの内容が正しい[0m
[32m      バリデーションエラーが表示される[0m
    ユーザのプロフィール情報編集失敗: nameを1文字にする
[32m      更新されない[0m
[32m      ユーザ編集画面を表示しており、フォームの内容が正しい[0m
[32m      バリデーションエラーが表示される[0m
    投稿データの新規投稿失敗: 投稿一覧画面から行い、titleを空にする
[32m      投稿が保存されない[0m
[32m      投稿一覧画面を表示している[0m
[32m      新規投稿フォームの内容が正しい[0m
[31m      バリデーションエラーが表示される (FAILED - 5)[0m
    投稿データの更新失敗: titleを空にする
[32m      投稿が更新されない[0m
[32m      投稿編集画面を表示しており、フォームの内容が正しい[0m
[32m      エラーメッセージが表示される[0m
  ログインしていない場合のアクセス制限のテスト: アクセスできず、ログイン画面に遷移する
[32m    ユーザ一覧画面[0m
[32m    ユーザ詳細画面[0m
[32m    ユーザ情報編集画面[0m
[32m    投稿一覧画面[0m
[32m    投稿詳細画面[0m
[32m    投稿編集画面[0m
  他人の画面のテスト
    他人の投稿詳細画面のテスト
      表示内容の確認
[32m        URLが正しい[0m

Failures:

  1) [STEP2] ユーザログイン後のテスト 投稿一覧画面のテスト 投稿成功のテスト 自分の新しい投稿が正しく保存される
     [31mFailure/Error: expect { click_button 'Create Book' }.to change(user.books, :count).by(1)[0m
     [31m  expected `Book::ActiveRecord_Associations_CollectionProxy#count` to have changed by 1, but was changed by 0[0m
     [36m# ./spec/system/02_after_login_spec.rb:99:in `block (4 levels) in <main>'[0m

  2) [STEP2] ユーザログイン後のテスト 投稿一覧画面のテスト 投稿成功のテスト リダイレクト先が、保存できた投稿の詳細画面になっている
     [31mFailure/Error: expect(current_path).to eq '/books/' + Book.last.id.to_s[0m
     [31m[0m
     [31m  expected: "/books/2"[0m
     [31m       got: "/books"[0m
     [31m[0m
     [31m  (compared using ==)[0m
     [36m# ./spec/system/02_after_login_spec.rb:103:in `block (4 levels) in <main>'[0m

  3) [STEP2] ユーザログイン後のテスト 自分の投稿詳細画面のテスト 投稿成功のテスト 自分の新しい投稿が正しく保存される
     [31mFailure/Error: expect { click_button 'Create Book' }.to change(user.books, :count).by(1)[0m
     [31m  expected `Book::ActiveRecord_Associations_CollectionProxy#count` to have changed by 1, but was changed by 0[0m
     [36m# ./spec/system/02_after_login_spec.rb:172:in `block (4 levels) in <main>'[0m

  4) [STEP3] 仕上げのテスト サクセスメッセージのテスト 投稿データの新規投稿成功時: 投稿一覧画面から行います。
     [31mFailure/Error: is_expected.to have_content 'successfully'[0m
     [31m  expected to find text "successfully" in "Bookers\nHome Users Books Log out\nUser info\nname m9vll3xa9g introduction 152b9g6edzh21hj2tkgk\nNew book\nTitle\nOpinion\nBooks一覧\nTitle Opinion\ncpz08\n6bt7bytv3ymneij0ds93\nok5cc\np3muhcer9vx9yik6lv88\nCopyRight Infratop.inc"[0m
     [36m# ./spec/system/03_finishing_touches_spec.rb:56:in `block (3 levels) in <main>'[0m

  5) [STEP3] 仕上げのテスト 処理失敗時のテスト 投稿データの新規投稿失敗: 投稿一覧画面から行い、titleを空にする バリデーションエラーが表示される
     [31mFailure/Error: expect(page).to have_content "can't be blank"[0m
     [31m  expected to find text "can't be blank" in "Bookers\nHome Users Books Log out\nUser info\nname 5okdhbn5sr introduction kihyc7j2i6k6xcy27hol\nNew book\nTitle\nOpinion\nBooks一覧\nTitle Opinion\n8qhfo\nbycqqwr8var21gzy1aiq\ntf8tu\nnb1n0b11ymsanihu0bwx\nCopyRight Infratop.inc"[0m
     [36m# ./spec/system/03_finishing_touches_spec.rb:146:in `block (4 levels) in <main>'[0m

Finished in 20.04 seconds (files took 1.16 seconds to load)
[31m160 examples, 5 failures[0m

Failed examples:

[31mrspec ./spec/system/02_after_login_spec.rb:98[0m [36m# [STEP2] ユーザログイン後のテスト 投稿一覧画面のテスト 投稿成功のテスト 自分の新しい投稿が正しく保存される[0m
[31mrspec ./spec/system/02_after_login_spec.rb:101[0m [36m# [STEP2] ユーザログイン後のテスト 投稿一覧画面のテスト 投稿成功のテスト リダイレクト先が、保存できた投稿の詳細画面になっている[0m
[31mrspec ./spec/system/02_after_login_spec.rb:171[0m [36m# [STEP2] ユーザログイン後のテスト 自分の投稿詳細画面のテスト 投稿成功のテスト 自分の新しい投稿が正しく保存される[0m
[31mrspec ./spec/system/03_finishing_touches_spec.rb:47[0m [36m# [STEP3] 仕上げのテスト サクセスメッセージのテスト 投稿データの新規投稿成功時: 投稿一覧画面から行います。[0m
[31mrspec ./spec/system/03_finishing_touches_spec.rb:144[0m [36m# [STEP3] 仕上げのテスト 処理失敗時のテスト 投稿データの新規投稿失敗: 投稿一覧画面から行い、titleを空にする バリデーションエラーが表示される[0m

