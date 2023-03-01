class BooksController < ApplicationController
  before_action :is_matching_login_user, only: [:edit, :update, :destroy]#edit,updateｱｸｼｮﾝ実行前にﾛｸﾞｲﾝ中ﾕｰｻﾞｰじゃない場合は一覧ページへﾘﾀﾞｲﾚｸﾄする
  def index
    @books = Book.all
    @book = Book.new#入力ﾌｫｰﾑ(部分テンプレへ渡す)
    @user = current_user#部分テンプレへ渡す
  end

  def create
    @book = Book.new(book_params)
    @book.user_id = current_user.id#ﾌｫｰﾑに入力済ﾃﾞｰﾀのﾕｰｻﾞidにﾛｸﾞｲﾝ中のﾕｰｻﾞｰid代入
    if @book.save                 #ﾃﾞｰﾀ保存されたら
      flash[:notice] = "You have created book successfully."#ﾌﾗｯｼｭﾒｯｾｰｼﾞ
      redirect_to book_path(@book.id)#詳細ﾍﾟｰｼﾞへ飛ぶ
    else                        #ﾃﾞｰﾀ保存されなかったら
      @books = Book.all
      #@book = Book.new#入力ﾌｫｰﾑ(部分テンプレへ渡す10行目で定義済だから不要)
      @user = current_user#部分テンプレへ渡す
      render :index#indexﾍﾟｰｼﾞを表示
    end
  end

  def show
    @book = Book.find(params[:id])#特定の投稿ﾃﾞｰﾀのid取得してレコードを取得
    @book_n = Book.new            #部分テンプレへ渡す
    @user = @book.user#部分テンプレへ渡す　投稿した人のﾕｰｻﾞ―情報
  end

  def destroy
    @book = Book.find(params[:id])
    @book.destroy
    redirect_to books_path
  end

  def edit
    @book = Book.find(params[:id])
  end

  def update
    @book = Book.find(params[:id])
    if @book.update(book_params)     #データ保存されたら
      flash[:notice] = "You have updated book successfully."
      redirect_to book_path(@book.id)
    else                             #データ保存されなかったら
      render :edit
    end
  end

  private
    def book_params
      params.require(:book).permit(:title, :body, :video)
    end

    def is_matching_login_user
      @book = Book.find(params[:id])#URLのid取得。そのidの投稿を取得
        unless @book.user == current_user#その投稿がﾛｸﾞｲﾝ中ﾕｰｻﾞｰのidと等しくない場合
          redirect_to books_path#一覧ページへリダイレクト
        end
    end

end
